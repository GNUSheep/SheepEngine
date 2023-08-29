const pieces = {
    p: "<div class=\"piece\"><i class=\"fa-solid fa-chess-pawn\"></i></div>" as keyof String, 
    r: "<div class=\"piece\"><i class=\"fa-solid fa-chess-rook\"></i></div>" as keyof String,
    n: "<div class=\"piece\"><i class=\"fa-solid fa-chess-knight\"></i></div>" as keyof String,
    b: "<div class=\"piece\"><i class=\"fa-solid fa-chess-bishop\"></i></div>" as keyof String,
    k: "<div class=\"piece\"><i class=\"fa-solid fa-chess-king\"></i></div>" as keyof String,
    q: "<div class=\"piece\"><i class=\"fa-solid fa-chess-queen\"></i></div>" as keyof String,

} 
const chess_board = document.getElementById("chess_board")
