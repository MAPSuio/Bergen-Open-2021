/*
Vi ønsker å lese noe input, anvende det på en algoritme, og produsere noe output.
Man importerer fmt, og leser input med fmt.Scanln() og skriver med fmt.Println()

Under følger et eksempel på en løsning til følgende problem:
"Gi meg summen av n heltall"
Input:
5
60
34
2
13
20
Output:
134
*/

package main

import fmt

func main(){
    var n int
    fmt.Scanln(&n)

    res := 0
    for i := 0; i < n; i++ {
        var m int
        fmt.Scanln(&m)
        res += m
    }
  
    fmt.Println(res)
}
