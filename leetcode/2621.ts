/*
Question:
Given a positive integer millis, write an asyncronous function that sleeps for millis milliseconds. It can resolve any value.
*/

async function sleep(millis: number): Promise<void> {
    return new Promise((resolve,reject) => {
        setTimeout(resolve,millis);
    })
}
