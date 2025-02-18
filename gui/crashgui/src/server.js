const express = require('express');
const spawn = require('child_process').spawn;
const path = require('path');

const app = express();
const port = 3000;

app.get('/list', (req, res) => {
  const path = req.query.path ? req.query.path : "";
  let process;

  // TODO: sanitize? maybe???
  process = spawn(
    "./src/binaries/cli", [...(path ? ["path", path] : []), "list"]
  );

  process.stdout.on( 'data', (data) => {
    console.log(`Received: ${data.slice(0, -1)}`);
    res.send(data.slice(0, -1));
  });

  process.stderr.on( 'data', (data) => {console.log(`Err: ${data}`)});
});

app.use(express.static(path.join(__dirname, '../build')));
app.get('/', function (req, res) {
  res.sendFile(path.join(__dirname, '../build', 'index.html'));
});

app.listen(port, () => {
  console.log(`Listening on port ${port}`)
})
