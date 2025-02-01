const express = require('express');
const spawn = require('child_process').spawn;
const app = express();
const port = 3001;

// let process = spawn(
//   "./binaries/cli", ["path", "tab", "list"]
// );

// process.stdout.on('data', (data) => {
//   console.log(`Received: ${data}`);
// });

// process.stderr.on('data', (data) => {
//   console.log(`Received: ${data}`);
// });

app.get('/list', (req, res) => {
  const path = req.query.path;
  let process;

  process = spawn(
    "./binaries/cli", ( path != undefined ? ["path", path, "list"] : ["list"] )
  );

  process.stdout.on( 'data', (data) => {
    console.log(`Received: ${data}`);
    res.send(data);
  });
})

app.listen(port, () => {
  console.log(`Listening on port ${port}`)
})
