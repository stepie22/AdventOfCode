const fs = require("fs");
const path = require("path");

module.exports.readInput = file => {
  return fs.readFileSync(path.join(__dirname, file)).toString();
};
