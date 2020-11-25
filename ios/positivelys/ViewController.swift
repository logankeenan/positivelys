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

protocol ViewControllerDelegate: class {
    func makeAppRequest(_ viewController: ViewController, request_as_json: String)
}

class ViewController: UIViewController, WKUIDelegate, WKScriptMessageHandler {

    weak var delegate: ViewControllerDelegate?
    var webView: WKWebView!

    private var html_markup: String!



    public convenience init(html_markup: String) {
        self.init()
        self.html_markup = html_markup
    }

    override func loadView() {
        let webConfiguration = WKWebViewConfiguration()
        webConfiguration.userContentController.add(self, name: "makeAppRequest")
        webView = WKWebView(frame: .zero, configuration: webConfiguration)
        webView.uiDelegate = self
        view = webView
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        webView.loadHTMLString(self.html_markup, baseURL: URL(string: AppService.hostName))
    }

    func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
        if message.name == "makeAppRequest" {
            self.delegate?.makeAppRequest(self, request_as_json: message.body as! String)
        }
    }
}
