from ast import List
import collections


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        diction = collections.defaultdict(list)
        for s in strs:
            count=[0]*26
            for c in s:
                count[ord(c)-ord("a")]+=1
            diction[tuple(count)].append(s)
        return diction.values()
# Time complexity: O(n*2)