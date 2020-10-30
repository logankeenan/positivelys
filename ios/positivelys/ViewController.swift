//
//  ViewController.swift
//  positivelys
//
//  Created by Logan Keenan on 10/30/20.
//
//

import UIKit


class ViewController: UIViewController {

    override func viewDidLoad() {
    super.viewDidLoad()
    // Do any additional setup after loading the view.
        
        let dirPaths = NSSearchPathForDirectoriesInDomains(.documentDirectory, .userDomainMask, true)

        let result = call_database(dirPaths[0] + "/database.sqlite")
        let query_result = String(cString: result!)
        call_database_free(UnsafeMutablePointer(mutating: result))
        print(query_result)
    }



}
