"""
Vi ønsker å lese noe input, anvende det på en algoritme, og produsere noe output.
Man kan lese inn med standard input(), og skrive med standard print()

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
"""

def main():
    n = int(input()) # Det første tallet, som markerer hvor mange tall som følger
    s = 0
    
    for _ in range(n): # Gjør noe med de neste n tallene
        sum += int(input())

    print(s)

if __name__ == "__main__":
   main()
