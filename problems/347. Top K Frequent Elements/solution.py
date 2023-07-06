from ast import List
import collections


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        diction = collections.defaultdict(int)
        for n in nums:
            diction[n]+=1
        result=[]
        while not len(result) >= k:
            max_value = max(diction.values())

        # Find the keys with the maximum value
            keys_with_max_value = [key for key, value in diction.items() if value == max_value]
            result+=keys_with_max_value
            for key in keys_with_max_value:
                diction.pop(key)
        return result
#Time complexity : O(n*k)
#I don't know how but beats 99.5% of the solutions