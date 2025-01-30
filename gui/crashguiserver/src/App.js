const express = require('express');
const spawn = require('child_process').spawn;
const app = express();
const port = 3001;

let process = spawn(
  "./cli", ["list"]
);

process.stdout.on('data', (data) => {
  console.log(`Received: ${data}`);
});


// app.get('/list', (req, res) => {
//   const path = req.query.path;

//   var process = spawn(
//     "./cli", ["list"]
//   );

//   process.stdout.on( (data) => {
//     console.log(`Received: ${data}`);
//     res.send(data);
//   });
// })

// app.listen(port, () => {
//   console.log(`Listening on port ${port}`)
// })
