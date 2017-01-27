import {Injectable} from "@angular/core"
import {Subject} from "rxjs"

@Injectable()
export class ThemeService {

	private toggleThemeSource = new Subject()

	toggleTheme$ = this.toggleThemeSource.asObservable()

	toggleTheme() {
		this.toggleThemeSource.next()
	}

}
