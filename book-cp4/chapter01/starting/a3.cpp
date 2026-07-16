//!UVa 11614 - Etruscan Warriors

#include <bits/stdc++.h>
using namespace std;
#define ll long long int

int main(){
    ios::sync_with_stdio(0);
    cin.tie(0);

    ll n;
    cin >> n;

    ll x;
    while(n--){
        cin >> x;
        ll ans = floor(((-1)+floor(sqrt(1+8*x)))/2);
        printf("%lld\n", ans);
    }
}