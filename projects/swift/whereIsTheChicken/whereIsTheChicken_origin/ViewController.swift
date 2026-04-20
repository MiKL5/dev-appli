//
//  ViewController.swift
//  whereIsTheChicken
//
//  Created by Mickael on 21/09/2024.
//

import UIKit

class ViewController: UIViewController {
    
    @IBOutlet weak var playtn: UIButton!
    @IBOutlet weak var scoreLbl: UILabel!
    @IBOutlet weak var chickenIV: UIImageView!
    @IBOutlet weak var resultLbl: UILabel!
    @IBOutlet weak var yesBtn: UIButton!
    @IBOutlet weak var noBtn: UIButton!
    
    var isGameOn = false
    var score: Int = 0
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
    }
    
    func play() {
        isGameOn = true
        updateUI()
    }
    
    func stop () {
        isGameOn = false
        updateUI()
    }
    
    func chickenAnswer(tag: Int) {
        //
    }
    
    func updateUI() {
        yesBtn.isHidden = !isGameOn
        noBtn.isHidden = !isGameOn
        playtn.setTitle(isGameOn ? "Démarrer" : "Stop", for: .normal)
        resultLbl.isHidden = !isGameOn
    }
    
    func updateScore () {
        scoreLbl.text = "score: \(score)"
    }
    
    func updateAnswer(success: Bool) {
        resultLbl.text = success ? "Gagné" : "Perdu"
    }
    
    
    @IBAction func playPressed(_ sender: Any) {
        isGameOn ? stop() : play()
    }
    
    
    @IBAction func answerPressed(_ sender: UIButton) {
    }
    
    

}

