import { exec } from "child_process";

const run = async (command: string): Promise<string> => {
  return new Promise((resolve, reject) => {
    exec(command, (err, stdout, stderr) => {
      if (stderr || err) {
        reject(stderr);
      } else {
        resolve(stdout);
      }
    });
  });
};

export default run;
