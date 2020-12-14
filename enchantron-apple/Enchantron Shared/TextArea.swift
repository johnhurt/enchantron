//
//  TextArea.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 8/6/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

class TextArea {
    var text: String = ""
    
    func setText(_ text: String) {
        self.text = text
    }
    
    func getText() -> String {
        return self.text
    }
}
