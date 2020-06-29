//
//  ProgressBar.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/16/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class ProgressBar : SKNode {
  
  let background = SKShapeNode()
  let barBackground = SKShapeNode()
  let bar = SKShapeNode()
  let labelNode = SKLabelNode()
  
  var barOrigin : CGPoint = CGPoint(x: 0, y: 0)
  var maxBarWidth : CGFloat = 0.0
  var barHeight : CGFloat = 0.0
  var progress : Int64 = 0
  
  override init() {
   
    background.fillColor = SKColor.clear
    background.strokeColor = SKColor.clear
    
    
    self.barBackground.fillColor = SKColor.lightGray
    self.barBackground.strokeColor = SKColor.clear
    
    self.bar.fillColor = SKColor.darkGray
    self.bar.strokeColor = SKColor.clear
    
    super.init()
    
    self.isUserInteractionEnabled = true
    
    addChild(self.background)
    addChild(self.barBackground)
    addChild(self.bar)
    
    labelNode.fontColor = SKColor.lightGray
    labelNode.horizontalAlignmentMode = SKLabelHorizontalAlignmentMode.center
    labelNode.verticalAlignmentMode = SKLabelVerticalAlignmentMode.center
    labelNode.fontName = "Indira K"
    
    
    addChild(labelNode)
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }
  
  func setSize(size: CGSize) {
    background.path = CGPath(
      rect: CGRect(origin: CGPoint(x: -size.width / 2, y: -size.height / 2), size: size),
      transform: nil)
    
    let interiorSize = CGSize(
      width: 0.95 * size.width,
      height: size.height - 0.05 * size.width)
    
    let barBackroundSize = CGSize(
      width: interiorSize.width,
      height: interiorSize.height / 3)
    
    self.barHeight = barBackroundSize.height / 3
    self.maxBarWidth = interiorSize.width - (self.barHeight * 2)
    self.barOrigin = CGPoint(
      x: -self.maxBarWidth / 2,
      y: -(interiorSize.height * 3.5 / 9))
    
    self.barBackground.path = CGPath(
      rect: CGRect(
        origin: CGPoint(x: -interiorSize.width / 2, y: -interiorSize.height * 1.5 / 3),
        size: barBackroundSize),
      transform: nil)
    
    labelNode.position = CGPoint(x: 0, y: interiorSize.height * 1 / 6)
    labelNode.fontSize = interiorSize.height / 4
    
    self.setIntValue(progress)
    
  }
  
  func setIntValue(_ value: Int64) {
    sleep(1)
    DispatchQueue.main.async {
      self.progress = value
      self.bar.path = CGPath(
          rect: CGRect(
              origin: self.barOrigin,
              size: CGSize(
                  width: self.maxBarWidth * CGFloat(value) / 100.0 ,
                  height: self.barHeight)),
        transform: nil)
    }
  }
  
  func getIntValue() -> Int64 {
    return self.progress
  }
  
  func setText(_ newText: String) {
    DispatchQueue.main.async {
      self.labelNode.text = newText
    }
  }
  
  func getText() -> String {
    return self.labelNode.text!
  }
}
