import {Command} from '@tauri-apps/plugin-shell';

export async function coreStart() {
    let result = await Command.create('exec-sh', [
        '-c',
        "echo 'Hello World!'",
    ]).execute();
    console.log(result);
}