//
//  ViewController.swift
//  positivelys
//
//  Created by Logan Keenan on 10/30/20.
//
//

import UIKit
import Foundation
import WebKit

class ViewController:  UIViewController, WKUIDelegate  {
    var webView: WKWebView!

    override func loadView() {
        let webConfiguration = WKWebViewConfiguration()
        webView = WKWebView(frame: .zero, configuration: webConfiguration)
        webView.uiDelegate = self
        view = webView
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.

        let request = "{\"body\":null,\"method\":\"GET\",\"path\":\"/positivelys\"}"
        let result = make_app_request(request)
        let query_result = String(cString: result!)

        make_app_request_free(UnsafeMutablePointer(mutating: result))

        let response: AppResponse? = try? JSONDecoder().decode(AppResponse.self, from: query_result.data(using: .utf8)!)
        webView.loadHTMLString((response?.body)!, baseURL: URL(string: "https://logankeenan.com"))
    }


}
