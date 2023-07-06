from ast import List


class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        def hv(i,j):
            return (board[i].count(board[i][j])<2)
        def vv(i,j):
            #Move from top to bottom keeping current element constant and k variable to see any dublicate of current element
            for k in range(9):
                if k==i:
                    continue
                current=board[i][j]
                if board[k][j]==current:
                    return False
            return True
        def gv(i,j):
            divI=i//3
            divJ=j//3
            grid_start_i=divI*3
            grid_start_j=divJ*3
            list=[]
            #Append all the elements in grid to list and check if count==1
            for k in range(grid_start_i,grid_start_i+3):
                for l in range(grid_start_j,grid_start_j+3):
                    list.append(board[k][l])
            return list.count(board[i][j])==1

        for i in range(9):
            for j in range(9):
                if(board[i][j]=="."):
                    continue
                #check horizontal validity
                if not hv(i,j):
                    return False
                #check vertical validity
                if not vv(i,j):
                    return False
                #check grid validity
                if not gv(i,j):
                    return False
        return True

#O(1) as board size is 9*9 (will always be)