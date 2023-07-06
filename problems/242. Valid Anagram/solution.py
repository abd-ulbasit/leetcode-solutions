class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s)!=len(t):
            return False
        s_hash,t_hash={},{}
        for i in range(len(s)):
            s_hash[s[i]]=1+s_hash.get(s[i],0)
            t_hash[t[i]]=1+t_hash.get(t[i],0)
        return s_hash==t_hash
        # Time complexity : O(n) n is length of string s/t
        # Space complexity : 0(n) n is no of characters in s/t 
