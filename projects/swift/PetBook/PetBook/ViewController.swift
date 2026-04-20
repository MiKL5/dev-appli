//
//  ViewController.swift
//  PetBook
//
//  Created by Mickael on 16/09/2024.
//

import UIKit

class ViewController: UIViewController {
    
    @IBOutlet weak var containerView: UIView!
    
    @IBOutlet var friendsUIV: [UIImageView]!
    
    override func viewDidLoad() {
        super.viewDidLoad()
        setupContainerView()
        //setupFriends()
    }
    func setupContainerView() {
        containerView.layer.cornerRadius = 55
        containerView.layer.opacity = 0.75
        containerView.layer.shadowColor = UIColor.black.cgColor
        containerView.layer.shadowRadius = 4
        containerView.layer.shadowOffset = CGSize(width: 0, height: -7)
        //containerView.layer.shadowOpacity = 0.05
    }
    func setupFriends() {
        friendsUIV.forEach { friend in
            let friendFrame = friend.frame
            let corner = friendFrame.height / 2
            friend.layer.cornerRadius = corner
            friend.layer.borderColor = UIColor.secondarySystemBackground.cgColor
            friend.layer.borderWidth = 2
        }
    }
}
