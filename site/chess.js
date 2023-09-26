//var game = new Chess('1nbqk2r/6pp/8/r7/3p4/3p2P1/5PKP/4q3 w k - 1 32')
// 8/3r4/1p2r2p/1k4pK/7P/4b3/6P1/8 b - - 0 54
var game = new Chess()

function onDragStart(square, piece) {
    var moves = game.moves({
        square: square,
        verbose: true
    })
    
    moves.forEach(square => {
        var square_obj = $('#chess_board .square-' + square.to)
        square_obj.addClass('highlight1-32417')
    });
}

function onDrop(source, target) {
    var move = game.move({
        from: source,
        to: target,
        promotion: 'q',
    })
    
    if (move === null) {    
        return 'snapback'
    }

    $.post("http://localhost:8080", game.fen(), (data, status) => {
        console.log(data)
        let from = data.slice(0, 2)
        let to = data.slice(2, 4)

        game.move({
            from: from,
            to: to,
            promotion: 'q',
        })
        chess_board.position(game.fen())
    });
}

function onSnapEnd () {
    chess_board.position(game.fen())
}

var config = {
    draggable: true,
    onDragStart: onDragStart,
    onDrop: onDrop,
    onSnapEnd: onSnapEnd,
    //position: '1nbqk2r/6pp/8/r7/3p4/3p2P1/5PKP/4q3',
    position: 'start',
}

var chess_board = Chessboard('chess_board', config)
$(window).resize(chess_board.resize)