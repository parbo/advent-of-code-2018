#include <list>
#include <vector>
#include <iostream>
#include <algorithm>

int main(int argc, char *argv[]) {
  auto players = std::atoi(argv[1]);
  auto marbles = std::atoi(argv[2]);
  std::list<int64_t> circle;
  int64_t max_score = 0;
  std::vector<int64_t> scores;
  scores.resize(players);
  circle.push_back(0);
  auto it = circle.begin();
  for (int marble = 1; marble < marbles + 1; ++marble) {
    if ((marble % 23) == 0) {
      for (int x = 0; x < 7; ++x) {
        if (it == circle.begin()) {
          it = circle.end();
        }
        --it;
      }
      auto value = *it;
      it = circle.erase(it);
      if (it == circle.end()) {
	it = circle.begin();
      }
      auto &score = scores[marble % players];
      score += (value + marble);
      max_score = std::max(score, max_score);
    } else {
      for (int x = 0; x < 2; ++x) {
        ++it;
        if (it == circle.end()) {
          it = circle.begin();
        }
      }
      it = circle.insert(it, marble);
    }
  }
  std::cout << max_score;
}
