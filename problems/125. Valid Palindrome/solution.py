class Solution:
    def isPalindrome(self, s: str) -> bool:
        if len(s)<2:
            return True
        string=s.lower()
        start,end=0,len(s)-1
        while start<end:
            while not string[start].isalnum() and start<end:
                start+=1
            while not string[end].isalnum() and end>start:
                end-=1
            if end>start:
                if not string[start]==string[end]:
                    return False
            start+=1
            end-=1
        return True
        #O(n)