// 1. Define the function that generates a key
function generateKey(): Promise<string> {
  return new Promise((resolve, reject) => {
	setTimeout(()=>{
        let chance = Math.random();
        if(chance <= 0.2){
            reject(new Error("Key generation failed"));
        }
        else{
            resolve("YOUR_KEY_HERE");
        }
    }, 2000);
    // # YOUR CODE GOES HERE:
    // - Use setTimeout to wait 2 seconds
    // - Use Math.random() to simulate a 20% failure rate
    // - If success → call resolve("YOUR_KEY_HERE")
    // - If fail → call reject("Error message")
  });
}

// 2. Define the function that requests a key
async function requestKey(): Promise<void> {
  try {
    const key:string = await generateKey();
    console.log(key)
    // # YOUR CODE GOES HERE:
    // - Call generateKey() using await
    // - Store it in a variable typed as string
    // - console.log the key if success
  } catch (error: any) {
    // # YOUR CODE GOES HERE:
    // - console.error the error message
    console.log("Error:", error.message)  //output: "Error: Key generation failed"
  }
}

// 3. Run the function to test
requestKey(); 
