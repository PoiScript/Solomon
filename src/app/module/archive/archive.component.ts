import {Component, Inject, OnInit} from '@angular/core'
import {Title} from '@angular/platform-browser'

import {PostService} from '../../service/post'
import {Intro} from '../../class/post'
import {SolomonConfig} from '../../interface/solomon-config'
import {CONFIG_TOKEN} from '../../config'
import {HeaderService} from '../../service/header/header.service'

@Component({
  selector: 'app-archive',
  templateUrl: './archive.component.html',
  styleUrls: ['./archive.component.scss']
})
export class ArchiveComponent implements OnInit {
  intros: Intro[]
  private BLOG_NAME: string

  constructor(private postService: PostService,
              private titleService: Title,
              private headerService: HeaderService,
              @Inject(CONFIG_TOKEN) config: SolomonConfig) {
    this.BLOG_NAME = config.BLOG_NAME
  }

  getArchive(): void {
    this.postService
      .getArchive()
      .then(intros => this.intros = intros)
  }

  ngOnInit() {
    this.headerService.changeHomeHeader('Archive')
    this.titleService.setTitle(`Archive - ${this.BLOG_NAME}`)
    this.getArchive()
  }

}
