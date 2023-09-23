var game = new Chess('1K6/6r1/1k6/8/8/8/8/3R4 w - - 0 1')

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
    })
    
    if (move === null) {    
        return 'snapback'
    }

    $.post("http://localhost:8080", game.fen(), (data, status) => {
        console.log(data)
        let from = data.slice(0, 2)
        let to = data.slice(2)

        game.move({
            from: from,
            to: to,
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
    position: '1K6/6r1/1k6/8/8/8/8/3R4',
}

var chess_board = Chessboard('chess_board', config)
$(window).resize(chess_board.resize)