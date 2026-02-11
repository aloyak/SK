import { spawn } from 'child_process';
import fs from 'fs';
import path from 'path';
import https from 'https';

const BINARY_URL = "https://github.com/aloyak/SK/releases/latest/download/SK";
const REMOTE_BINARY_PATH = "/tmp/SK";
const LOCAL_BINARY_PATH = path.join(process.cwd(), 'SK');

export default async function handler(req, res) {
  try {
    const activePath = fs.existsSync(LOCAL_BINARY_PATH) ? LOCAL_BINARY_PATH : REMOTE_BINARY_PATH;

    if (activePath === REMOTE_BINARY_PATH && !fs.existsSync(REMOTE_BINARY_PATH)) {
      await downloadFile(BINARY_URL, REMOTE_BINARY_PATH);
    }

    fs.chmodSync(activePath, '755');

    if (req.method === 'GET') {
      const child = spawn(activePath, ['--version']);
      let result = '';
      child.stdout.on('data', (data) => result += data);
      child.stderr.on('data', (data) => result += data);
      child.on('close', () => res.status(200).send(result.trim() || "SK Interpreter Ready"));
      return;
    }

    if (req.method === 'POST') {
      const { code, inputs = [] } = JSON.parse(req.body);
      const tempFile = path.join('/tmp', `code-${Date.now()}.sk`);
      fs.writeFileSync(tempFile, code);

      const child = spawn(activePath, [tempFile, '--safe']);
      let output = '';
      let error = '';

      if (inputs.length > 0) {
        inputs.forEach(val => child.stdin.write(val + "\n"));
        child.stdin.end();
      }

      child.stdout.on('data', (data) => output += data);
      child.stderr.on('data', (data) => error += data);

      const timeout = setTimeout(() => {
        child.kill();
        if (!res.writableEnded) res.status(200).send(output + error + "\n[Execution Timed Out]");
      }, 5000);

      child.on('close', () => {
        clearTimeout(timeout);
        if (fs.existsSync(tempFile)) fs.unlinkSync(tempFile);
        if (!res.writableEnded) res.status(200).send(output + error);
      });
    }
  } catch (err) {
    res.status(500).send(`Server Error: ${err.message}`);
  }
}

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    https.get(url, (res) => {
      if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
        return downloadFile(res.headers.location, dest).then(resolve).catch(reject);
      }
      const file = fs.createWriteStream(dest);
      res.pipe(file);
      file.on('finish', () => {
        file.close();
        resolve();
      });
    }).on('error', reject);
  });
}