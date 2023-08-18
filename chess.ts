function isDigit(string) {
    return /^[0-9]*$/.test(string);
}

function create_chess_board() {
    for (let i = 0; i < 64; i++) {
        const square = document.createElement('div');
        square.classList.add('square');
        Math.floor((i/8)+i) % 2 == 0 ? square.classList.add('white') : square.classList.add('brown');
        chess_board?.append(square)
    }

    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";    
    let fen_pieces = fen.split(" ")[0];
    fen_pieces.split("/").forEach((row, row_index) => {
        row.split('').forEach((piece, piece_index) => {
            let square = chess_board?.getElementsByClassName('square')[piece_index+row_index*8];
            if (square == undefined) {
                console.error("Square can't be found", piece_index+row_index*8);
            } else {
                isDigit(piece) ? {} : square.innerHTML = pieces[piece.toLowerCase()];
                square?.getElementsByTagName('svg')[0] != undefined && piece == piece.toUpperCase() ? 
                    square.getElementsByTagName('svg')[0].classList.add('white_piece') : {};
            }
        })
    });
}

window.onload = function(){create_chess_board()};
