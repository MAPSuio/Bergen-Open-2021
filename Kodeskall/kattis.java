/*
Vi ønsker å lese noe input, anvende det på en algoritme, og produsere noe output.
Man kan løse dette ved å importere Scanner og så ta System.in som parameter.
Deretter kan man blant annet lese Strings med scanner.readNextLine() og ints med scanner.nextInt()
Du kan finne alt på https://docs.oracle.com/javase/7/docs/api/java/util/Scanner.html

For output skal man ikke returne noe. Vi ønsker bare å printe, vanlig System.out.println()

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

import java.util.Scanner;

class Kattis {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        
        int n = sc.nextInt(); // Det første tallet, som markerer hvor mange tall som følger
        int s = 0;
        
        for (int i = 0; i < n; i++) { // Gjør noe med de neste n tallene
            s += nextInt();
        }
        
        System.out.println(s);
        sc.close();
    }
}
