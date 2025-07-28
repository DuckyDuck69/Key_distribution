/**
 * Import function triggers from their respective submodules:
 *
 * import {onCall} from "firebase-functions/v2/https";
 * import {onDocumentWritten} from "firebase-functions/v2/firestore";
 *
 * See a full list of supported triggers at https://firebase.google.com/docs/functions
 */

import * as admin from "firebase-admin";
import * as crypto from "crypto";

import {onCall, HttpsError} from "firebase-functions/v2/https";
//const {logger} = require("firebase-functions/v2");


// Start writing functions
// https://firebase.google.com/docs/functions/typescript

// For cost control, you can set the maximum number of containers that can be
// running at the same time. This helps mitigate the impact of unexpected
// traffic spikes by instead downgrading performance. This limit is a
// per-function limit. You can override the limit for each function using the
// `maxInstances` option in the function's options, e.g.
// `onRequest({ maxInstances: 5 }, (req, res) => { ... })`.
// NOTE: setGlobalOptions does not apply to functions using the v1 API. V1
// functions should each use functions.runWith({ maxInstances: 10 }) instead.
// In the v1 API, each function can only serve one request per container, so
// this will be the maximum concurrent request count.


// export const helloWorld = onRequest((request, response) => {
//   logger.info("Hello logs!", {structuredData: true});
//   response.send("Hello from Firebase!");
// });

// Initialize firebase admin
if (!admin.apps.length) {
  admin.initializeApp();
}

// v1
export const getKey = onCall(
  async (request:any) => {
    // VERIFY AUTHENTICATION
    const uid = request.auth?.uid ||"testUser1234";
    if (!uid) {
      throw new HttpsError("unauthenticated", "Login required");
    }
    
    //CHECK IF USER ALREADY HAS A KEY
    // get the document from Firestore: /keys/{uid}
    const docs = await admin.firestore().collection("keys").doc(uid).get();
    if (docs.exists) {
      return docs.data(); // return an object {}
    }

    //GENERATE A NEW KEY
    else {
      const newKey = crypto.randomBytes(16).toString("hex");
      await admin.firestore().collection("keys").doc(uid).set({
        key: newKey,
        createdAt: Date.now(), //TODO: get the Firebase's server time
        status: "new_key_created"
      });
      return {key: newKey, status: "new_key_created"};
    }
  }
);

//TO DO: How many times can user revoke key per day? What if the Firestore write fails? 
// (=> catch the error and throw a proper HttpsError)
export const revokeKey = onCall(
  async (request:any) =>{
    const uid = request.auth?.uid ||"testUser123";
    if(!uid){
      throw new HttpsError("unauthenticated", "Login required")
    }
    //CHECK IF USER ALREADY HAS A KEY
    const docs = await admin.firestore().collection("keys").doc(uid).get();
    if (!docs.exists) { //if the user dont have a key, create new
      const newKey = crypto.randomBytes(16).toString("hex");
      await admin.firestore().collection("keys").doc(uid).set({
        key: newKey,
        createdAt:  Date.now(), 
        status: "new_key_created"
      });
      return {key: newKey, status: "new_key_created"};
    }
    //REVOKE THEIR KEY
    else{
      const revoke = crypto.randomBytes(16).toString("hex");
      await admin.firestore().collection("keys").doc(uid).set({
        key: revoke,
        revokedAt:  Date.now(),
        status: "revoked_and_renewed"
      });
      return {key: revoke, status: "revoked_and_renewed"} 
    }
  }
);
