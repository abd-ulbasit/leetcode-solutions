from ast import List

class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        myset=set()
        for n in nums:
            if n in myset:
                return True
            myset.add(n)
        return False
        #O(n)