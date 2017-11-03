const fs = require('fs')

function asyncFunc() {
  return new Promise(
    function (resolve, reject) {
      fs.readdir('.', function(err, succ) {
        if (err) { reject(err) }
        resolve(succ)
      })
    }
  );
}

a = asyncFunc()
console.log(a) // -> Promise { <pending> }
a.then(res => console.log(res))
  .catch(e => console.log(e))
