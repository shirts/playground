import awscala._, s3._
import scala.collection.mutable.ArrayBuffer

object Main extends App {
  implicit val s3 = S3.at(Region.NorthernVirginia)
  val buckets: Seq[Bucket] = s3.buckets

  println("Enter bucket name (press enter for all)")
  var findThisBucket = scala.io.StdIn.readLine
  var foundBuckets = new ArrayBuffer[String]

  // Look for any s3 buckets containing the string input from user
  for ( bucket <- buckets ) {
    val bucketName = bucket.name
    if ( bucketName contains findThisBucket ) {
      foundBuckets.append(bucketName)
    }
  }

  // Output message indicating what's been found
  if ( foundBuckets.isEmpty ) {
    println("No buckets found")
  } else {
    println(f"Found the following buckets containing $findThisBucket")
  }

  // Output each bucket found
  for ( bucketName <- foundBuckets ) {
    println(bucketName)
  }
}
