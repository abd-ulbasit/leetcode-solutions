from ast import List

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        result=[]
        for i in range(len(nums)-2):
            if nums[i]>0:
                break
            l,r=i+1,len(nums)-1
            while l<r:
                currentSum=nums[i]+nums[l]+nums[r]
                if currentSum>0:
                    r-=1
                elif currentSum<0:
                    l+=1
                else:
                    triplet=[nums[i],nums[l],nums[r]]
                    if result.count(triplet)==0:
                        result.append(triplet)
                    l+=1
                    r-=1
                    while nums[l]==nums[l-1] and l<r:
                        l+=1
        return result