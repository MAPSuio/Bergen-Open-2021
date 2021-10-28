/*
Vi ønsker å lese noe input, anvende det på en algoritme, og produsere noe output.
Man kan lese inn med standard cin >>, og skrive med standard cout <<

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

#include <iostream>
using namespace std;

int main() {
  int sum = 0, n, tmp;
  cin >> n; // Det første tallet, som markerer hvor mange tall som følger
  
  for (int i = 0; i < n; i++) { // Gjør noe med de neste n tallene
    cin >> tmp;
    sum += tmp;
  }
  
  cout << sum;
  return 0;
}
