function isDigit(string) {
    return /^[0-9]*$/.test(string);
}
function create_chess_board() {
    for (let i = 0; i < 64; i++) {
        const square = document.createElement('div');
        square.classList.add('square');
        square.addEventListener('dragstart', dragStart);
        square.addEventListener('dragover', dragOver);
        square.addEventListener('drop', drop);
        square.setAttribute('square_index', i.toString());
        Math.floor((i / 8) + i) % 2 == 0 ? square.classList.add('white') : square.classList.add('brown');
        chess_board?.append(square);
    }
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fen_pieces = fen.split(" ")[0];
    fen_pieces.split("/").forEach((row, row_index) => {
        row.split('').forEach((piece, piece_index) => {
            let square = chess_board?.getElementsByClassName('square')[piece_index + row_index * 8];
            if (square == undefined) {
                console.error("Square can't be found", piece_index + row_index * 8);
            }
            else {
                if (!isDigit(piece)) {
                    square.innerHTML = pieces[piece.toLowerCase()];
                    square?.getElementsByClassName('fa-solid')[0] != undefined && piece == piece.toUpperCase() ?
                        square.getElementsByClassName('fa-solid')[0].classList.add('white_piece') : {};
                    square?.getElementsByClassName('fa-solid')[0] != undefined ? square.getElementsByClassName('fa-solid')[0].setAttribute('draggable', 'true') : {};
                    square?.getElementsByClassName('fa-solid')[0] != undefined ? square.getElementsByClassName('fa-solid')[0].setAttribute('piecetype', piece.toLowerCase()) : {};
                }
            }
        });
    });
}
window.onload = function () { create_chess_board(); };
let movingPiece;
let moves = new Array(0);
let side_to_move = 1;
function generateValidMoves(piece, startPosition, piece_classList) {
    let piece_color = piece_classList.contains('white_piece') ? "w" : "b";
    switch (piece) {
        case 'p':
            let squares = chess_board.getElementsByClassName('square');
            if ((startPosition >= 8 && startPosition <= 15 && piece_color == "b") || (startPosition >= 48 && startPosition <= 55 && piece_color == "w")) {
                squares[startPosition - 16 * side_to_move].firstElementChild == null ? moves.push((startPosition - 16 * side_to_move).toString()) : {};
            }
            let taken_left = chess_board?.getElementsByClassName('square')[startPosition - 9 * side_to_move].firstElementChild?.firstElementChild;
            if (taken_left != undefined) {
                taken_left.classList != movingPiece.classList ? moves.push((startPosition - 9 * side_to_move).toString()) : {};
            }
            let taken_right = chess_board?.getElementsByClassName('square')[startPosition - 7 * side_to_move].firstElementChild?.firstElementChild;
            if (taken_right != undefined) {
                taken_right.classList != movingPiece.classList ? moves.push((startPosition - 7 * side_to_move).toString()) : {};
            }
            squares[startPosition - 8 * side_to_move].firstElementChild == null ? moves.push((startPosition - 8 * side_to_move).toString()) : {};
            break;
        default:
            break;
    }
}
function dragStart(e) {
    if (e.target.localName != "i") {
        return;
    }
    movingPiece = e.target;
    moves = [];
    generateValidMoves(e.target.attributes["piecetype"].value, Number(e.target.parentNode.parentNode.attributes["square_index"].value), e.target.classList);
}
function dragOver(e) {
    e.preventDefault();
}
function drop(e) {
    e.stopPropagation();
    if (movingPiece == null || e.target == movingPiece) {
        return;
    }
    if (e.target.localName == "i") {
        if (!moves.includes(e.target.parentNode.parentNode.attributes["square_index"].value)) {
            return;
        }
        ;
        e.target.parentNode.appendChild(movingPiece);
        e.target.remove();
    }
    else {
        if (!moves.includes(e.target.attributes["square_index"].value)) {
            return;
        }
        ;
        e.target.append(movingPiece.parentNode);
    }
}
