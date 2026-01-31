import { exec } from 'child_process';
import fs from 'fs';
import path from 'path';
import https from 'https';

const BINARY_URL = "https://github.com/AlmartDev/SK/releases/download/0.6.1/SK";
const REMOTE_BINARY_PATH = "/tmp/SK";
const LOCAL_BINARY_PATH = path.join(process.cwd(), 'SK');

export default async function handler(req, res) {
  try {
    let activePath = REMOTE_BINARY_PATH;

    if (fs.existsSync(LOCAL_BINARY_PATH)) {
      activePath = LOCAL_BINARY_PATH;
    } else if (!fs.existsSync(REMOTE_BINARY_PATH)) {
      await downloadFile(BINARY_URL, REMOTE_BINARY_PATH);
    }

    fs.chmodSync(activePath, '755');

    if (req.method === 'GET') {
      exec(`${activePath} --version`, (err, stdout, stderr) => {
        const result = stdout || stderr;
        res.status(200).send(result.trim() || "SK Interpreter Ready");
      });
      return;
    }

    if (req.method === 'POST') {
      const tempFile = path.join('/tmp', `code-${Date.now()}.sk`);
      fs.writeFileSync(tempFile, req.body);

      exec(`${activePath} ${tempFile}`, (err, stdout, stderr) => {
        if (fs.existsSync(tempFile)) fs.unlinkSync(tempFile);
        
        const combinedOutput = stdout + stderr;
        
        if (err && !combinedOutput) {
          return res.status(200).send(`Process Error: ${err.message}`);
        }

        res.status(200).send(combinedOutput || "Execution finished (no output).");
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