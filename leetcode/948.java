/*
Question:
You start with an initial power of power, an initial score of 0, 
and a bag of tokens given as an integer array tokens, 
where each tokens[i] donates the value of tokeni.

Your goal is to maximize the total score by strategically playing these tokens. 
In one move, you can play an unplayed token in one of the two ways (but not both for the same token):
    Face-up: If your current power is at least tokens[i], you may play tokeni, losing tokens[i] power and gaining 1 score.
    Face-down: If your current score is at least 1, you may play tokeni, gaining tokens[i] power and losing 1 score.

Return the maximum possible score you can achieve after playing any number of tokens.
*/

// Approach:
// To maximize profit, we should always try to spend the lowest power to get the highest score and vice versa.
// So we can sort the tokens and then use two pointers to keep track of the score and power.
// If we can spend power to get score, we do it with the minimum token and increase the score.
// Otherwise, we can spend score to get power from the maximum token and decrease the score.
// At the end, we return the maxScore.

class Solution {
  public int bagOfTokensScore(int[] tokens, int power) {
    // Sort the tokens
    Arrays.sort(tokens);

    // Initialize left and right pointers
    int left = 0;
    int right = tokens.length - 1;

    // Track score and maxScore
    int score = 0;
    int maxScore = 0;

    // Loop until the pointers meet
    while(left < tokens.length && right >=0 && left <= right) {
      if(tokens[left] <= power) {
        // We can increase the score
        power -= tokens[left++];
        maxScore = Math.max(maxScore, ++score);
      } else if(score > 0) {
        // We can get more power
        power += tokens[right--];
        score--;
      } else {
        // Return the maxScore
        return maxScore;
      }
    }
    // Return the maxScore
    return maxScore;    
  }
}
