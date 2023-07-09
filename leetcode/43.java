/*
Question:
Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
*/

// Approach:
// 1) We make pos to store the result string digits as an int[]
// 2) Then we do a double loop over the digits of both the given strings, going from right to left for each one
//    The index of sum of the product of two digits will be i+j+1(since we are counting from the right) and of the carry will be i+j
//    But we also have to cosider the value aready there at those indices.
//    Then we multiply the digits at the indices, add them to the existing value at the sum position
//    Then we set i+j+1 index of pos to the sum and i+j index to the carry of the calculated number
// 3) After the loop, we need to convert the pos int[] to a string, also removing the leading spaces
//    We make a string builder to do this and a bool called leading to track if the zeros are considered leading zeros or not
//    Then we go voer the pos array and if the digit is 0 as well as leading and not the last digit, we continue to the next iteration
//    We continue if it's not the last digit because if it's the last digit then we wanna add it 
//    even it it's 0 and considered leading because otherwise the ans string will be empty
// 4) At the end we return the string builder as a string

class Solution {
    public String multiply(String num1, String num2) {
        int l1 = num1.length(), l2 = num2.length();
        int[] pos = new int[l1 + l2];

        for(int i=l1-1; i>=0; i--) {
            for(int j=l2-1; j>=0; j--) {
                int prod = (num1.charAt(i) - '0') * (num2.charAt(j) - '0');
                int sum = pos[i+j+1] + prod;
                pos[i+j+1] = sum%10;
                pos[i+j] += sum/10;
            }
        }

        StringBuilder ans = new StringBuilder();
        boolean leading = true;
        for(int i=0; i<pos.length; i++){
            if(pos[i]==0 && i<pos.length-1 && leading)continue;
            if(leading)leading = false;
            ans.append(String.valueOf(pos[i]));
        }
        return ans.toString();
    }
}
