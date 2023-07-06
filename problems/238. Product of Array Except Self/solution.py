from ast import List
class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        res=[0]*len(nums)
        ##handle 2 zeros case
        if nums.count(0)>=2:
            return res
        prefix_prod=1
        for i in range(len(nums)):
            res[i]=prefix_prod
            prefix_prod*=nums[i]
        postfix_prod=1
        for i in range(len(nums)-1,-1,-1):
            res[i]*=postfix_prod
            postfix_prod*=nums[i]
        return res