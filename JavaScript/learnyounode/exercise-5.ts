'use strict'
import * as fs from 'fs'
import * as path from 'path'

class FileLister {
  directory: string
  fileExtension: string

  constructor(directory: string, fileExtension: string) {
    this.directory = directory
    this.fileExtension = fileExtension
  }

  listFilesInDirectory() {
    fs.readdir(this.directory, (err, fileNames) => {
      fileNames.forEach(fileName => {
        if (this.hasCorrectExtension(fileName)) {
          console.log(fileName)
        }
      })
   })
  }

  private hasCorrectExtension(fileName: string) {
    if (path.extname(fileName) === this.fileExtension) {
      return true
    }
  }
}

const directory = process.argv[2]
const fileExtension = process.argv[3]
let fileLister = new FileLister(directory, fileExtension)
fileLister.listFilesInDirectory()
