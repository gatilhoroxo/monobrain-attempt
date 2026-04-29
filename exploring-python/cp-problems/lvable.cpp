#include <bits/stdc++.h>

#define lld long long int 

using namespace std;

void insert(string& s, int i, char k){
    auto it = s.begin() + i;
    s.insert(it, k);
}
void remove(string& s, int i){
    s.erase(i, 1);
}
void swap(string& s, int i, int j){
    char temp = s[i];
    s[i] = s[j];
    s[j] = temp;
}
void reverso(string& s, int i, int j){
    auto it_i = s.begin() + i;
    auto it_j = s.begin() + j;
    reverse(it_i, it_j);
}

lld mfind(string& s, string sub){
    auto pos = s.find(sub);
    if(pos != string::npos)
        return pos;
    else return -1;
}

int main(){
    ios::sync_with_stdio(0);
    cin.tie(0);

    int n;
    string palavra;
    cin >> n >> palavra;
    //cout << n << ' ' << palavra << '\n';

    int ans = 0;
    lld pos = mfind(palavra, "lv");
    if(pos != -1){
        cout << ans << '\n';
        return 0;
    }
    //cout << "temos que manipular então\n";
    lld lpos = mfind(palavra,"l");
    lld vpos = mfind(palavra,"v"); 
    ans++;

    if(lpos != -1 || vpos != -1){
        cout << ans << '\n';
        return 0;
    }

    ans++;
    cout << ans << '\n';

    return 0;
}