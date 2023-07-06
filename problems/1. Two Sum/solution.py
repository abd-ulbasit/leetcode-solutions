from ast import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hash_table={}
        for i in range(len(nums)):
            if (hash_table.get(nums[i])!=None):
                return [i,hash_table[nums[i]]]
            else:
                hash_table[target-nums[i]]=i
    
    # time complexity  : O(n)
    # space complexity : O(n)