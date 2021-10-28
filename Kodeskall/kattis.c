/*
  Under er et eksempel på en "vanlig" flyt i C.
  Først leser man inn et tall: n, som forteller hvor mange tall som kommer.
  Deretter leser man inn verdier, her en serie med integere, som man lagrer i et array: arr.
  Deretter gjør man beregninger(utelatt), før man printer resultatet.
*/

#include <stdio.h>
int main() {
    int n;
    int arr[n];
    scanf("%d", &n);
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }
    int out = 0;
    printf("%d\n", out);
}
