//
//  ViewController.swift
//  ElleEstOuLaPoulette
//
//  Created by matthieu passerel on 19/08/2021.
//

import UIKit

class ViewController: UIViewController {
    
    @IBOutlet weak var playBtn: UIButton!
    @IBOutlet weak var scoreLbl: UILabel!
    @IBOutlet weak var pouleIV: UIImageView!
    @IBOutlet weak var resultLabel: UILabel!
    @IBOutlet weak var yesBtn: UIButton!
    @IBOutlet weak var noBtn: UIButton!
    
    
    var currentChicken: Int = 0
    var isGameOn = false
    var score: Int = 0
    let wrongAnswers: [Int] = [1, 3, 6] // tableau de mauvaises féponses
    var chickenCenter: CGPoint?
    

    override func viewDidLoad() {
        super.viewDidLoad()
        updateUI()
        resultLabel.text = ""
    }
    
    func play() {
        isGameOn = true
        score = 0
        updateScore()
        updateUI()
        setupChicken()
    
    }
    
    func stop() {
        isGameOn = false
        updateUI()
        pouleIV.image = UIImage(named: "bg")
    }
    

    func setupChicken() {
        currentChicken = Int.random(in: 0...7) // image au hasard
        let imageString = "poule\(currentChicken)"
        let image = UIImage(named: imageString)
        pouleIV.image = image
    }
    
    func updateUI() {
        yesBtn.isHidden = !isGameOn
        noBtn.isHidden = !isGameOn
        playBtn.setTitle(!isGameOn ? "Jouer" : "Arrêter", for: .normal)
        resultLabel.isHidden = !isGameOn
        pouleIV.layer.cornerRadius = 50
        pouleIV.isUserInteractionEnabled = !isGameOn
    }
    
    func updateScore() {
        scoreLbl.text = "Score: \(score)"
    }
    
    func updateAnswer(success: Bool) {
        resultLabel.text = success ? "Gagné" : "Perdu"
    }
    
    func checkAnswer(tag: Int) {
        let wrong = wrongAnswers.contains(currentChicken)
        let saidNo = tag == 1
        let success = wrong == saidNo
        if success {
            score += 1
            updateScore()
        }
        updateAnswer(success: success)
        setupChicken()
        setupBG(color: .systemBackground)
    }
    
    func leftOrRight(x: CGFloat) -> Int {
        let viewX = self.view.center.x
        let valueX = viewX - x
        print(valueX)
        if valueX > 25 {
            return 0
        } else if valueX < -25 {
            return 1
        } else {
            return 2
        }
    }
    
    func setupBG(color: UIColor) {
        self.view.backgroundColor = color
    }
    
    func getTouch(touches: Set<UITouch>) -> CGPoint? {
        guard let touch = touches.first else { return nil }
        let center = touch.location(in: self.view)
        return center
    }
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard chickenCenter == nil else { return }
        chickenCenter = pouleIV.center
    }
    
    
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let center = getTouch(touches: touches) else { return }
        let y = chickenCenter!.y
        let x = center.x
        pouleIV.center = CGPoint(x: x, y: y)
        let tag = leftOrRight(x: center.x)
        switch tag {
        case 0: setupBG(color: .systemGreen)
        case 1: setupBG(color: .systemRed)
        default: setupBG(color: .systemBackground)
        }
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let center = getTouch(touches: touches) else { return }
        pouleIV.center = chickenCenter!
        let tag = leftOrRight(x: center.x
        )
        if tag != 2 {
            checkAnswer(tag: tag)
        }
    }
    
    

    
    @IBAction func playBtnPressed(_ sender: Any) {
        isGameOn ? stop() : play()
    }
    
    
    @IBAction func answerPressed(_ sender: UIButton) {
        let tag = sender.tag
        checkAnswer(tag: tag)
    }
}

