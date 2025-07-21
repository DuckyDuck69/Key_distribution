import { initializeApp } from "firebase/app";
import { getFunctions, httpsCallable, connectFunctionsEmulator } from "firebase/functions";

// load firebase prj id 
const firebaseConfig = {
  projectId: "key-distribution-d7a45", 
};

//initialize Firebase App
const app = initializeApp(firebaseConfig);

//connect to the Emulator 
const functions = getFunctions(app);
connectFunctionsEmulator(functions, "localhost", 5001);

//call the function

const revokeKey = httpsCallable(functions, "revokeKey");

revokeKey({})
  .then((result) => {
    console.log("Function Result:", result.data);
  })
  .catch((error) => {
    console.error("Function Error:", error);
  });

