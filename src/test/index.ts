import { init, spawn, terminate } from "./process";

// async function sleep(time: number) {
//     console.log('x');
//     await new Promise<void>(resolve => {
//         setInterval(() => {
//             console.log('y');
//             resolve();
//         }, 2000);
//     });
// }

async function main() {
    // await sleep(100).then(() => {
    //     console.log('z');
    //     process.exit(0);
    // });
    // let subprocess = await spawn('dfx', ['start']);
    // // subprocess.stdout.on('data', d => {
    // //     console.log(`[dfx] ${d}`);
    // // });

    // // subprocess.stderr.on('data', (d) => {
    // //     console.log(`[dfx] ${d}`);
    // // });

    // // subprocess.on('error', e => {
    // //     console.log(`[dfx] ${e}`);
    // // });

    // // subprocess.on('exit', (c, s) => {
    // //     console.log(`[dfx] ${c} ${s}`);
    // // });

    // console.log('[parent] started');
    // // process.on('SIGINT', () => {
    // //     console.log('[parent] SIGINT', subprocess.kill());
    // // });

    // // process.on('exit', e => {
    // //     // subprocess.kill();
    // //     console.log(`[parent] exiting ${e}`);
    // // })

    // await new Promise((resolve, reject) => {
    //     setTimeout(resolve, 10 * 1000);
    // });

    // throw "HELLO";

    // console.log('[parent] awake');

    // // subprocess.kill();
    // await terminate();
    // console.log('[parent] killed');
}

main();