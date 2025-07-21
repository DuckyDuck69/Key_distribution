/**
 * Import function triggers from their respective submodules:
 *
 * import {onCall} from "firebase-functions/v2/https";
 * import {onDocumentWritten} from "firebase-functions/v2/firestore";
 *
 * See a full list of supported triggers at https://firebase.google.com/docs/functions
 */

import * as functions from "firebase-functions";
import * as admin from "firebase-admin";
import * as crypto from "crypto";



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

// v2
export const getKey = functions.https.onCall(
  async (data: any, context: any) => {
    // VERIFY AUTHENTICATION
    const uid = context.auth?.uid ||"testUser123";
    if (!uid) {
      throw new functions.https.HttpsError("unauthenticated", "Login required");
    }

    // 2.CHECK IF USER ALREADY HAS A KEY
    // get the document from Firestore: /keys/{uid}
    const docs = await admin.firestore().collection("keys").doc(uid).get();
    if (docs.exists) {
      return docs.data(); // return an object {}
    }

    // 3. GENERATE A NEW KEY IF NOT EXIST
    else {
      const newKey = crypto.randomBytes(16).toString("hex");
      await admin.firestore().collection("keys").doc(uid).set({
        key: newKey,
        createdAt: Date.now(),
      });
      return {key: newKey, createdAt: Date.now()};
    }
  }
);
