//!UVa 11614 - Etruscan Warriors

#include <bits/stdc++.h>
using namespace std;

int main(){
    ios::sync_with_stdio(0);
    cin.tie(0);

    int n;
    cin >> n;

    int x;
    while(n--){
        cin >> x;
        int ans=0;
        for(int i=1; (x >= 0); i++){
            if((x-i) >= 0){
                ans++;
            }
            x-=i;
        }

        printf("%d\n", ans);
    }
}