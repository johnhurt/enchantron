//
//  SystemView.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/19/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

class SystemView {
    let resourceLoader : ResourceLoader
    
    init(resourceLoader : ResourceLoader) {
        self.resourceLoader = resourceLoader
    }
    
    func getResourceLoader() -> ResourceLoader {
        return self.resourceLoader
    }
}
