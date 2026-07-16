#include <bits/stdc++.h>
using namespace std;

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(0);

    bool check=false;
    while(!check){
        int v=0,t=0;
        if(scanf("%d %d", &v, &t)==EOF) check = true;
        if(!check)printf("%d\n", 2*v*t);
    }
}