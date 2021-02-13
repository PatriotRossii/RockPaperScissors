available_moves = ["Rock", "Paper", "Sciccors"]
win_table = {
    "Rock": "Sciccors", "Paper": "Rock", "Sciccors": "Paper"
}

def validate_move(move):
    return move in available_moves

def check_move(move):
    if not validate_move(move):
        print("Invalid move")

hint = ", ".join(available_moves)
while True:
    first_opponent_move = input(f"First player, enter your move ({hint}): ")
    check_move(first_opponent_move)

    second_opponent_move = input(f"Second player, enter your move ({hint}): ")
    check_move(second_opponent_move)

    if first_opponent_move == second_opponent_move:
        print("Chess")
    elif win_table[first_opponent_move] == second_opponent_move:
        print("First player won")
    else:
        print("Second player won")