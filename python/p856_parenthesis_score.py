# https://leetcode.com/problems/score-of-parentheses/
class Solution:
    def scoreOfParentheses(self, s: str) -> int:
        # Cheese solution using eval :)
        return eval(s.replace("()", "+1").replace("(", "+2*("))