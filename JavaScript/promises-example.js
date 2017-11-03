const fs = require('fs')

//
// one promise
//

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

//
// multiple promises
//

function asyncFunc3() {
  return new Promise(
    function (resolve, reject) {
      fs.readdir('.', function(err, succ) { // this is async
        return reject(new Error('err asyncFunc3'))
        if (err) { reject(err) }
        resolve('asyncFunc3')
      })
    }
  );
}


function asyncFunc2() {
  return new Promise(
    function (resolve, reject) {
      fs.readdir('.', function(err, succ) { // this is async
        if (err) { reject(err) }
        resolve('asyncFunc2')
      })
    }
  );
}


function asyncFunc1() {
  return new Promise(
    function (resolve, reject) {
      fs.readdir('.', function(err, succ) { // this is async
        if (err) { reject(err) }
        resolve('asyncFunc1')
      })
    }
  );
}

Promise.all([
  asyncFunc1(),
  asyncFunc2(),
  asyncFunc3()
]).then(([res1, res2, res3]) => {
  console.log(res1)
  console.log(res2)
}).catch(e => console.log(e))

