/*
  Under er et eksempel på en kodeflyt i Kotlin.
  Denne leser inn `n`, og så leser den inn n linjer, der hver linje
  er en serie med integere som er separert av mellomrom.
*/

fun main() {
    val n = readLine()!!.toInt()
    var res = 0

    for (i in 0..n-1) {
        var line = readLine()!!.split(" ").toMutableList().map(String::toInt)
        // Gjør ting
    }
    println(res)
}
